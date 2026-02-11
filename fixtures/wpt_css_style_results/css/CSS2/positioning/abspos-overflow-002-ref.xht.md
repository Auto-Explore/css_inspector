# css/CSS2/positioning/abspos-overflow-002-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/abspos-overflow-002-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  body
  {
  line-height: 1.25;
  margin-left: 0;
  }

  div#positioned
  {
  background: green;
  color: white;
  right: 0;
  position: absolute;
  top: 0;
  width: 10em;
  }

  p {margin-top: 36px;}

  div#overflow
  {
  height: 8em;
  overflow: scroll;
  width: 8em;
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
