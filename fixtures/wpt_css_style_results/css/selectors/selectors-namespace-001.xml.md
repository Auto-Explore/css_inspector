# css/selectors/selectors-namespace-001.xml

```json
{
  "format_version": 3,
  "file": "css/selectors/selectors-namespace-001.xml"
}
```

## style[0]

```css

    @namespace html url(http://www.w3.org/1999/xhtml);
    @namespace test url(http://www.example.org/);
    html|body { color: red; }
    [test] { color: green; }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
