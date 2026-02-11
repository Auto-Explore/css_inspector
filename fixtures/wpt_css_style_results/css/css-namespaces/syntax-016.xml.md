# css/css-namespaces/syntax-016.xml

```json
{
  "format_version": 3,
  "file": "css/css-namespaces/syntax-016.xml"
}
```

## style[0]

```css

    @namespace test url("test");
    @media all {
      test|test { background-color: lime; }
    }
    @namespace test2 url("test");
    test2|test { background: red }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
