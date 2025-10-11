# Case study of the Ebou application

- Uses the "Swift Composable Architecture" style. Manages app domain and process into:

	- **State**
	- **Action**
	- **Reducer**
	- **Store**


## How does ui get displayed?

Manual trace from `main()` function:

1. Root app rendering function: 

	Contextual definitions:
	```
	let environment_state = use_state(cx, || {
		let model = if let Some(user) = user {
			Model::new(user.instance_url.clone(), Some(user.token_access_token))
		} else {
			Model::default()
		};

		Environment::new(model, repository.get().clone())
	});
	let environment = environment_state.get();
	...
	let should_show_login = use_state(cx, || !has_user);
	```

	Rendering:
	```
	cx.render(rsx! {
		environment.model.has_token.then(||
			rsx!(crate::components::loggedin::LoggedInApp {
				environment: environment_state,
				should_show_login: should_show_login,
			})
		),
		should_show_login.then(|| rsx!(crate::components::login::LoginApp {
			environment: environment_state,
			should_show_login: should_show_login
		}))
	})
	```

	(a) if the user has an access token render the LoggedInApp
	(b) if the user is not logged in / present render a different LoginApp


2, 

	(a) Login App

		- `view.rs` hierarchly splits the ui into a collection of different component functions,
			sends user input from ui to backend using the 'viewstore' ex:

			```
			onclick: move |_| {
				view_store.send(LoginAction::SelectInstance(Selection::Instance(x.clone())))
			}
			```

	(b) Logged in app
