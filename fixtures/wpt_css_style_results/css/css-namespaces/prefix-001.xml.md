# css/css-namespaces/prefix-001.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/prefix-001.xml"
}
```

## style[0]

```css

   @namespace Foo "y";
   @namespace foo "x";
   test { background:red }
   Foo|test { background:lime }
   foo|test { background:red }
   FOO|test { background:red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
