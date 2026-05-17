# Resource Synthesis and Variation Protocol (RSVP) Specification

Resource Synthesis and Variation Protocol (RSVP) extends the \[Internationalized\] Uniform Resource Identifier protocol to allow for the querying combinations of networked resources and structured data

`https://fanneispanker.com/armedescorts/hardboiledwhore/&genre=(mdo:Genre::fiction:crime:noir*mdo:Genre::fiction:comedy:sex_comedy)`

`#Columbo&audience=(mdo:Media::)`

The form of an RSVP Identifier is:

```ebnf

IRI := SCHEME AUTHORITY? PATH? QUERY_LIST? FRAGMENT?
SCHEME := SCHEME_NAME ":"
AUTHORITY := "//"? AUTHORITY_NAME
PATH := ("/" IDENTIFIER)* "/"?
QUERY_LIST := "?" QUERY ("&" QUERY)*
QUERY := KEY ("=" VALUE)?
KEY := IDENTIFIER
VALUE := IDENTIFIER
FRAGMENT := "#" IDENTIFIER
IDENTIFIER := SAFE_CHAR+

RSVPI := RESOURCE (RELATION_EXPRESSION)*
RESOURCE := IRI
OPERATION := {UNION | INTERSECTION | RELATION}
UNION := "+"
INTERSECTION := "*"
RELATION_EXPRESSION := "&" RELATION RESOURCE
RELATION := VERB from VOCABULARY

```

Where `IRI` is an Internationalized Resource Identifier as defined within RFC 3987.


```
._~:/?#[]@!$&'()*+,;=
```