# css/CSS2/margin-padding-clear/margin-collapse-115-ref.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/margin-padding-clear/margin-collapse-115-ref.xht"
}
```

## style[0]

```css
<![CDATA[
  table
  {
  border: solid 3px;
  border-spacing: 0;
  font-size: 50px;
  }

  td {padding: 0;}

  div {width: 2em;}

  .a {border-top: yellow solid 1em;}

  .b {border-top: orange solid 2em;}

  .c {border-top: lime solid 1em;}

  .d {border-top: aqua solid 1em;}
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
