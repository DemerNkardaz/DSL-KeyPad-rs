pub fn add(left: u64, right: u64) -> u64 {
	left + right
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let result = add(2, 2);
		assert_eq!(result, 4);
	}
}

// Использовать ipc handler для общения между растом и браузерным окном

// use wry::webview::WebViewBuilder;

// let webview = WebViewBuilder::new(window)?
//     .with_ipc_handler(|_window, payload| {
//         println!("JS sent: {}", payload);
//     })
//     .build()?;

// JS:

// window.ipc.postMessage(JSON.stringify({
//     type: "settings:update",
//     data: {
//         theme: "dark",
//         fontSize: 14
//     }
// }));
