# css/css-display/run-in/run-in-contains-relpos-block-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/run-in-contains-relpos-block-001.xht"
}
```

## style[0]

```css
<![CDATA[
    div { display: block; }
    .run-in { display: run-in; font-weight: bold }
    #target { border: 2px solid black; }
    .run-in div { position: relative; top: 2em; left: 3em; }
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
