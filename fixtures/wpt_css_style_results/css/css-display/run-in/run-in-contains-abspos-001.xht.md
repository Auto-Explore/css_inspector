# css/css-display/run-in/run-in-contains-abspos-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/run-in-contains-abspos-001.xht"
}
```

## style[0]

```css
<![CDATA[
    #container { position: relative; top: -1em; }
    div { display: block; }
    .run-in { display: run-in; font-weight: bold }
    #target { border: 2px solid black; position: relative; top: 1em; }
    .run-in > span { position: absolute; top: 0 }
    .run-in > span + span { position: static; visibility: hidden; }
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
