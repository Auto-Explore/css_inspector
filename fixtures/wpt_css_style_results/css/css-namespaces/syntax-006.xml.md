# css/css-namespaces/syntax-006.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-006.xml"
}
```

## style[0]

```css

   @namespace x u\00072l("test");
   @import url("support/fail.css");
   @namespace url("test2");
   x|test { background:lime }
   test { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
