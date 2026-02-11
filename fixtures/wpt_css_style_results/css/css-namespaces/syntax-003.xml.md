# css/css-namespaces/syntax-003.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-003.xml"
}
```

## style[0]

```css

   *|test { background:red }
  
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

   @namespace url(test-a);
   test { background:lime }
  
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

   @namespace url('test-b');
   test { background:lime }
  
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

   @namespace url("test-c");
   test { background:lime }
  
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

   @namespace 'test-d';
   test { background:lime }
  
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

   @namespace "test-e";
   test { background:lime }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
