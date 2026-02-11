# css/css-display/run-in/run-in-contains-inline-block-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/run-in-contains-inline-block-001.xht"
}
```

## style[0]

```css
<![CDATA[
    div { display: block; }
    .run-in { display: run-in; font-weight: bold }
    #target { border: 2px solid black; }
    .run-in > span { display: inline-block }
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
