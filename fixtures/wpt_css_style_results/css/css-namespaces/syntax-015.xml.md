# css/css-namespaces/syntax-015.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-015.xml"
}
```

## style[0]

```css

   @namespace x url("test");
   @namespace x url("}x&lt; >x{");
   t { background:lime }
   x|t { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
