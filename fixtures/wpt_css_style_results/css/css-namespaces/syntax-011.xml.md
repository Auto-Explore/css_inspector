# css/css-namespaces/syntax-011.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-011.xml"
}
```

## style[0]

```css

   @namespace url("http://example.com/");
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

   @namespace url("http://example.com/");
   *|t2 { background:lime }
   t2 { background:red }
  
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

   @namespace url("HTTP://example.com/");
   *|t3 { background:lime }
   t3 { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

   @namespace url("http://example.COM/");
   *|t4 { background:lime }
   t4 { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

   @namespace url("%41");
   *|t5 { background:lime }
   t5 { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[5]

```css

   @namespace url("A");
   *|t6 { background:lime }
   t6 { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
