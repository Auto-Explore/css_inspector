# css/css-writing-modes/percent-padding-vlr-005.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/percent-padding-vlr-005.xht"
}
```

## style[0]

```css
<![CDATA[
  div.outer
    {
      background-color: blue;
      border-top: blue solid 3px;
      border-right: blue solid 3px;
      border-bottom: blue solid 2px;
      border-left: blue solid 3px;
      height: 160px;
      writing-mode: vertical-lr;
    }

  div.inner
    {
      background-color: transparent;
      height: 50px;
    }

  div.foo
    {
      padding-bottom: 0%; /* 0px */
      padding-left: 31.25%; /* 31.25% mult by 160px == 50px */
      padding-right: 0%; /* 0px */
      padding-top: 55.625%; /* 20px + 50px + 19px == 89px ; 89px divided by 160px == 55.625% */
    }

  div.bar
    {
      padding-bottom: 0%; /* 0px */
      padding-left: 0%; /* 0px */
      padding-right: 31.25%; /* 31.25% mult by 160px == 50px */
      padding-top: 12.5%; /* 12.5% mult by 160px == 20px */
    }

  div#reference
    {
      margin-top: 1em;
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
