#[cfg(test)]
use dashmap::DashMap;
#[cfg(test)]
use once_cell::sync::Lazy;

#[cfg(test)]
static ACTUAL: Lazy<DashMap<usize, String>> = Lazy::new(|| DashMap::default());

macro_rules! println {
    ($($arg:tt)*) => {
        #[cfg(not(test))]
        std::println!($($arg)*)

        // 通常時はprintln!をそのまま呼び出す
        #[cfg(test)]
        {
            let formatted_message = format!($($arg)*); // テスト時にはメッセージをフォーマット
            // フォーマットされたメッセージを利用する。例えば、標準出力に出力する、ファイルに記録する、アサーションで検証するなど。
            std::println!("TestOutput: {}", formatted_message) // 例: テスト出力用に加工したメッセージを出力
        }
    };
}

// マクロの使用例
fn main() {}

#[cfg(test)]
mod tests {
	#[test]
	fn test_conditional_return() {
		let mut a = once_cell::sync::OnceCell::<i32>::new();
		let b = a.set(20);
	}
}
