# css/css-namespaces/syntax-014.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-014.xml"
}
```

## style[0]

```css

   t, t2, t3 { background:red }
  
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

   @import x {}
   @namespace x "test";
   x|t { background:lime }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

   @namespace x "test-top";
   @foobar this is funny { not:sure }
   @namespace "test";
   @foobar this is funner;
   t2 { background:lime }
   x|t3 { background:lime }
   
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
