# css/css-namespaces/syntax-010.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-010.xml"
}
```

## style[0]

```css

   @namespace "1";
   @namespace dummy "yummy";
   @namespace "2";
   *|t { background:lime }
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

   @namespace "1";
   @namespace dummy "yummy";
   @namespace "2";
   *|t2 { background:red }
   t2 { background:lime }
  
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

   @namespace x "1";
   @namespace dummy "yummy";
   @namespace x "2";
   *|t3 { background:red }
   x|t3 { background:lime }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
