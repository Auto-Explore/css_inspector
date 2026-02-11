# css/css-namespaces/prefix-006.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/prefix-006.xml"
}
```

## style[0]

```css

   @namespace "test";
   |t { background:lime }
   t { background:red }
  
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

   @namespace "test";
   t2 { background:lime }
   |t2 { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
