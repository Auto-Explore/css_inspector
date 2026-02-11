# css/css-namespaces/syntax-012.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-012.xml"
}
```

## style[0]

```css

   test { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

   @namespace/* test */
   a
   url(
test-a );

   @namespace/**/b/**/url(	'test-b'
);

   @namespace	c	url("test-c"
);

   a|test, b|test, c|test { background:lime }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
