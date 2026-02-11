# css/css-namespaces/syntax-013.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-013.xml"
}
```

## style[0]

```css

   t, t2, t3, t4, t5 { background:red }
  
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

   @namespace "test" {}
   t { background:lime }
  
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
@namespace x "test
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

   @namespace "fail;
   ; t3 { background:lime }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[4]

```css

   @namespace url('fail);
     t4 { background:red !important; }
   );
   t4 { background:lime }
  
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unbalanced braces.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[5]

```css

   @namespace url(test);
   @namespace url('test' x);
   t5 { background:lime }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
