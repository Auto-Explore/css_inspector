# css/css-display/run-in/run-in-contains-inline-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-display/run-in/run-in-contains-inline-003.xht"
}
```

## style[0]

```css
<![CDATA[
    div { display: block; }
    .run-in { display: run-in; font-weight: bold }
    #target { border: 2px solid black; position: relative; }
    .run-in > span { visibility: hidden; } /* tests that the abs pos actually
                                              shows up */
    .run-in > span > span { display: table-cell; visibility: visible; }
    .run-in > span > span > span { position: absolute; top: 0; }
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
