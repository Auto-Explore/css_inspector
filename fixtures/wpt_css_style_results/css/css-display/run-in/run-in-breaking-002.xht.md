# css/css-display/run-in/run-in-breaking-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/run-in-breaking-002.xht"
}
```

## style[0]

```css
<![CDATA[
    div { display: block; }
    .run-in { display: run-in; border: 5px solid blue; font-weight: bold;
              direction: rtl; }
    #target { border: 2px solid black; padding: 1em; direction: rtl; }
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
