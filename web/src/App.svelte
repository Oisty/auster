<script>
  import Logo from "./Logo.svelte";
  import { Route, Router } from "svelte-routing";
  import SignIn from "./SignIn.svelte";
  import Debug from "./debug/Components.svelte";

  export let url = "";

  //TODO Check if user is authenticated
  //TODO if not authenticated route to sign in
  //TOTO else just show the UI lol
</script>

<Router {url}>
  <Route path="signin" component={SignIn} />
  <Route path="debug/components" component={Debug} />
</Router>
