vcl 4.0;

backend pokeapi {
  .host = "pokeapi.co";
  .ssl = 1;
	.ssl_sni = 1;
	.ssl_verify_peer = 1;
	.ssl_verify_host = 1;
}

backend translations {
  .host = "api.funtranslations.com";
  .ssl = 1;
	.ssl_sni = 1;
	.ssl_verify_peer = 1;
	.ssl_verify_host = 1;
}

sub vcl_recv {
  if (req.http.host = "pokemon") {
    set req.backend_hint = pokemon;
  } else {
    set req.backend_hint = translations;
  }
}
