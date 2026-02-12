# css/CSS2/bidi-text/bidi-breaking-003-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/bidi-text/bidi-breaking-003-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  p + .set {border-top: solid orange;}

  .set
  {
  border-bottom: solid orange;
  clear: both;
  float: left;
  }

  .control
  {
  border: silver solid;
  color: blue;
  float: left;
  font: bold larger monospace;
  margin: 1em;
  padding: 0.25em;
  }

  /* ensure BDO processing */
  bdo
  {
  direction: ltr;
  unicode-bidi: bidi-override;
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
