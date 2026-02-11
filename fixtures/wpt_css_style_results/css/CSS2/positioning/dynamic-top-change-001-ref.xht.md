# css/CSS2/positioning/dynamic-top-change-001-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/dynamic-top-change-001-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  p
  {
  line-height: 1.25;
  margin: 1em 0;
  }

  strong {line-height: 1;}

  div
  {
  background-color: green;
  height: 100px;
  margin-top: 112px;
  /*
  The red square appears 96px below the p margin box:
    2em : grandparent's top (dynamically set)
  +
    2em : parent's top (inherited)
  +
    2em : #red's top (inherited)
  =======
    96px
  max(margin-bottom of p, margin-top of div)
  max(16px, margin-top of div) == 16px + 96px;
  therefore, margin-top of div == 16px + 96px
  */
  width: 100px;
  }
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
