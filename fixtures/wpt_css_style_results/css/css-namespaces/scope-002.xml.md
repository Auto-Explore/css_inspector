# css/css-namespaces/scope-002.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/scope-002.xml"
}
```

## style[0]

```css

   test { background:lime }
  
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

   @import url("support/scope-002a.css");
   @import url("support/scope-002b.css");
   @namespace w "test";
   x|test { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
