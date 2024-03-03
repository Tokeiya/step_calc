use std::collections::VecDeque;
use std::io::Write;

use anyhow::Result as AnyResult;

use parser::infix as Infix;
use parser::rpn::parser as Rpn;
use syntax::arithmetic_expression::ArithmeticExpression;
use syntax::binary_operation::Operation;
use syntax::expression::Expression;
use syntax::number_value::NumberValue;

fn token_to_string(scr: &VecDeque<Rpn::Token>) -> String {
    let mut buff = String::default();

    for elem in scr.iter().rev() {
        match elem {
            Rpn::Token::Number(num) => match num {
                NumberValue::Integer(i) => buff.push_str(&i.to_string()),
            },
            Rpn::Token::Operator(op) => match op {
                Operation::Add => buff.push('+'),
                Operation::Sub => buff.push('-'),
                Operation::Mul => buff.push('*'),
                Operation::Div => buff.push('/'),
            },
        }

        buff.push(' ');
    }

    let len = buff.len();

    if len != 0 {
        buff.remove(buff.len() - 1);
    }
    buff
}

fn write_header(input: &VecDeque<Rpn::Token>, writer: &mut dyn Write) -> AnyResult<()> {
    let title = token_to_string(&input);

    _ = writer.write(
        br#"<!DOCTYPE html>
<html lang="ja">

<head>
    <meta charset="UTF-8">
    <style>

        .step {
            margin-bottom: 40px;
            border-bottom: 2px solid black;
        }


        table, th, td {
            border: 1px solid black;
            border-collapse: collapse;
        }

        table{
            margin-bottom: 10px;
        }

        th{
            padding: 5px;
            text-align: center;
        }

        td {
            padding: 5px;
            text-align: left;
        }

        h2 {
            margin-top: 5px;
            margin-bottom: 3px;
            border-bottom: 5px solid black;
        }

        h3 {
            margin-top: 0;
            margin-bottom: 20px;
            border-bottom: 3px solid darkgray;
        }
    </style>
    <title>"#,
    )?;

    writer.write_fmt(format_args!("{title}"))?;

    _ = writer.write(
        br#"</title>
</head>
<body>"#,
    )?;

    Ok(())
}

fn write_expression(expression: &Expression, writer: &mut dyn Write, calc: bool) -> AnyResult<()> {
    let expr = Infix::formatter::minimal_infix_notation(expression)
        .replace('{', "(")
        .replace('}', ")");

    if calc {
        let ans = expression.calc()?;
        let ans = match ans {
            NumberValue::Integer(i) => i.to_string(),
        };

        writer.write_fmt(format_args!("{} = {}", expr, ans))?
    } else {
        writer.write_fmt(format_args!("{expr}"))?;
    }

    Ok(())
}

fn write_state(
    recent: &str,
    input: &VecDeque<Rpn::Token>,
    stack: &Vec<Expression>,
    writer: &mut dyn Write,
) -> AnyResult<String> {
    let formula = token_to_string(&input);

    _ = writer.write(
        br#"<div class="step">
<h2>"#,
    )?;

    writer.write_fmt(format_args!("{}", &formula))?;

    _ = writer.write(b"</h2>\n");

    writer.write_fmt(format_args!(
        "<h3>Recent:{recent}</h3>\n<table>\n<tr>\n<th>TOP</th>\n</tr>\n"
    ))?;

    let mut iter = stack.iter().rev();

    if let Some(expr) = iter.next() {
        match expr {
            Expression::Number(_) => {
                _ = writer.write(b"<tr>\n<td>\n")?;
                write_expression(expr, writer, false)?;
                _ = writer.write(b"\n</td>\n</tr>\n")?;
            }
            Expression::Bracket(_) => unreachable!(),
            Expression::BinaryOperation(_) => {
                _ = writer.write(b"<tr>\n<td>\n")?;
                write_expression(expr, writer, true)?;
                _ = writer.write(b"\n</td>\n</tr>\n")?;
            }
        }
    }

    for elem in iter {
        _ = writer.write(b"<tr>\n<td>\n")?;
        write_expression(elem, writer, false)?;
        _ = writer.write(b"\n</td>\n</tr>\n")?;
    }

    _ = writer.write(
        br##"<tr>
<th>Bottom</th>
</tr>
</table>
</div>
"##,
    )?;

    Ok(formula)
}

fn write_footer(writer: &mut dyn Write) -> AnyResult<()> {
    _ = writer.write(
        br#"</body>
</html>"#,
    )?;
    Ok(())
}

pub fn write_html<T: std::io::Write>(input: &str, mut writer: T) -> AnyResult<()> {
    let (mut stream, remainder) = Rpn::tokenize(input);

    if !remainder.trim().is_empty() {
        return Err(anyhow::Error::msg("Tokenize error"));
    }

    let mut stack = Vec::<Expression>::default();

    write_header(&stream, &mut writer)?;
    let mut recent = token_to_string(&stream);

    loop {
        recent = write_state(&recent, &stream, &stack, &mut writer)?;

        if !Rpn::step_calc(&mut stream, &mut stack) {
            break;
        }
    }

    write_footer(&mut writer)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;
    
    use syntax::expression::Expression;
    
    use crate::test_helper::{assert_text, TrimOption};
    
    use super::*;
    
    fn create_cursor() -> Cursor<Vec<u8>> {
        Cursor::<Vec<u8>>::default()
    }
    fn gen_token_stream() -> VecDeque<Rpn::Token> {
        let (ret, rem) = Rpn::tokenize("4 2 3 4 5 / + * -");
        assert!(rem.is_empty());
        ret
    }

    #[test]
    fn token_to_string_test() {
        let tokens = gen_token_stream();
        let act = token_to_string(&tokens);

        assert_eq!(&act, "4 2 3 4 5 / + * -");
    }

    #[test]
    fn write_header_test() {
        const EXPECTED: &str = r#"<!DOCTYPE html>
<html lang="ja">

<head>
    <meta charset="UTF-8">
    <style>

        .step {
            margin-bottom: 40px;
            border-bottom: 2px solid black;
        }


        table, th, td {
            border: 1px solid black;
            border-collapse: collapse;
        }

        table{
            margin-bottom: 10px;
        }

        th{
            padding: 5px;
            text-align: center;
        }

        td {
            padding: 5px;
            text-align: left;
        }

        h2 {
            margin-top: 5px;
            margin-bottom: 3px;
            border-bottom: 5px solid black;
        }

        h3 {
            margin-top: 0;
            margin-bottom: 20px;
            border-bottom: 3px solid darkgray;
        }
    </style>
    <title>4 2 3 4 5 / + * -</title>
</head>
<body>"#;
        let mut cursor = create_cursor();
        write_header(&gen_token_stream(), &mut cursor).unwrap();

        let act = String::from_utf8(cursor.into_inner()).unwrap();

        println!("{}", &act);

        assert_text(&act, EXPECTED, Some(&[TrimOption::Both]), true)
    }

    #[test]
    fn write_footer_test() {
        const EXPECTED: &str = r#"</body>
</html>"#;

        let mut cursor = create_cursor();
        write_footer(&mut cursor).unwrap();

        let act = String::from_utf8(cursor.into_inner()).unwrap();

        assert_text(&act, EXPECTED, Some(&[TrimOption::Both]), true);
    }

    #[test]
    fn write_state_test() {
        const EXPECTED: &str = r#"<div class="step">
    <h2>3 4 5 / + * -</h2>
    <h3>Recent:2 3 4 5 / + * -</h3>
    <table>
        <tr>
            <th>TOP</th>
        </tr>
        <tr>
            <td>
                2
            </td>
        </tr>
        <tr>
            <td>
                4
            </td>
        </tr>
        <tr>
            <th>Bottom</th>
        </tr>
    </table>
</div>"#;

        let mut stream = gen_token_stream();
        let mut stack = Vec::<Expression>::default();

        assert!(Rpn::step_calc(&mut stream, &mut stack));
        assert!(Rpn::step_calc(&mut stream, &mut stack));
        let mut cursor = create_cursor();
        write_state("2 3 4 5 / + * -", &stream, &stack, &mut cursor).unwrap();

        let act = String::from_utf8(cursor.into_inner()).unwrap();
        assert_text(&act, EXPECTED, Some(&[TrimOption::Both]), true);
    }
}
