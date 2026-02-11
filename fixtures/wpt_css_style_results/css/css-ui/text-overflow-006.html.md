# css/css-ui/text-overflow-006.html

```json
{
  "format_version": 3,
  "file": "css/css-ui/text-overflow-006.html"
}
```

## style[0]

```css

div {
  white-space: pre;
  font-family: monospace;
  text-overflow: ellipsis;
  overflow: hidden;
  width: 9.5ch;
  /* 9ch ought to be enough,
     but Safari seems to have aliasing issues that make the ellipsis character larger than 1ch by a bit.
     Adding an extra .5 does not change the validity of the test,
     and lets safari fit “PASS…” in the space provided.
     This issue may be a bug, but if so, it is unrelated to what this test is testing,
     so no need to force a fail when an easy workaround is available.
   */
}
span { float: right; }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
