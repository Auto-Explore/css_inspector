# css/css-flexbox/overflow-auto-001.html

```json
{
  "format_version": 3,
  "file": "css/css-flexbox/overflow-auto-001.html"
}
```

## style[0]

```css

/* Make it possible for red to appear in webkit/blink browsers. But the test
 * still works in other browsers, you'll just get a grey scrollbar that
 * shouldn't be there.
 */
::-webkit-scrollbar-track, ::-webkit-scrollbar-thumb, ::-webkit-scrollbar {
  width: 10px;
  background: red;
}

#flex {
  display: flex;
}

#item {
  height: 300px;
  overflow: auto;
  scrollbar-color: red;
}

#child {
  height: 600px;
}
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
