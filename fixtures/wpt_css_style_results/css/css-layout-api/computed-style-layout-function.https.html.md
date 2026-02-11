# css/css-layout-api/computed-style-layout-function.https.html

```json
{
  "format_version": 3,
  "file": "css/css-layout-api/computed-style-layout-function.https.html"
}
```

## style[0]

```css

#test1 { display: layout(test1); }
#test2 { display: layout(); }
#test3 { display: layout(test3, invalid); }
#test4 { --display: layout(test4); display: var(--display); }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
