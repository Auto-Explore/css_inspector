# css/css-namespaces/scope-001.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/scope-001.xml"
}
```

## style[0]

```css

   @namespace x url("test");
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

   x|test { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
