# css/CSS2/bidi-text/bidi-breaking-002.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/bidi-text/bidi-breaking-002.xht"
}
```

## style[0]

```css
<![CDATA[
     /* Make test easier to read */
    .test, .control {
      color: blue;
      font: bold larger monospace;
      margin: 1em;
      padding: 0.25em;
      border: solid silver;
      float: left;
    }
    .set {
      clear: both;
      float: left;
      border-bottom: solid orange;
    }
    p + .set {
      border-top: solid orange;
    }

    /* ensure BDO processing */
    bdo {
      unicode-bidi: bidi-override;
      direction: ltr;
    }

    /* Enable preservation of source line breaks
       (and PS and LS for certain nonconformant ws-collapsing implementations) */
    .pre {
      white-space: pre; white-space: pre-lines;
    }
  ]]>
```

```json
{
  "errors": 4,
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
    },
    {
      "message": "Invalid value for property “border”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
