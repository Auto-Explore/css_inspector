# css/css-display/run-in/run-in-inherit-001.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/run-in-inherit-001.xht"
}
```

## style[0]

```css
<![CDATA[
    div { display: block; }
    .run-in { display: run-in; font-weight: bold; border: inherit}
    #target { border: 2px solid black; color: black; margin: 2em; padding: 10px }
    #container { color: green; border: 10px outset orange; }
  ]]>
```

```json
{
  "errors": 3,
  "messages": [
    {
      "message": "Invalid input.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
