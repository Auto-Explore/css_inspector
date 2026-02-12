# css/CSS2/positioning/dynamic-top-change-002-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/dynamic-top-change-002-ref.xht"
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
  margin-top: 28px;
  /*
  The green square appears 12px below the p margin box:
  2em + 2em - (16px + 20px + 16px) + 16px
  max(margin-bottom of p, margin-top of div)
  max(16px, margin-top of div) == 16px + 12px;
  therefore, margin-top of div == 16px + 12px
  */
  width: 100px;
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
