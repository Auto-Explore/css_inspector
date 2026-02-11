# css/css-namespaces/syntax-009.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-009.xml"
}
```

## style[0]

```css

   @namespace x "test";
   test { background:red }
   x|test { background:lime }
   x\00007Ctest { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
