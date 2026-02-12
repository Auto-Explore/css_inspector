# css/CSS2/bidi-text/bidi-breaking-001.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/bidi-text/bidi-breaking-001.xht"
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
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
