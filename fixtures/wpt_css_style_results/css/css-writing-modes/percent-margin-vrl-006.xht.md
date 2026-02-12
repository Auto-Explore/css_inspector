# css/css-writing-modes/percent-margin-vrl-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/percent-margin-vrl-006.xht"
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
      writing-mode: vertical-rl;
    }

  div.inner
    {
      background-color: yellow;
      height: 50px;
      width: 50px;
      writing-mode: horizontal-tb;
    }

  div.foo
    {
      margin-bottom: 0%; /* 0px */
      margin-left: 0%; /* 0px */
      margin-right: 31.25%; /* 31.25% mult by 160px == 50px */
      margin-top: 12.5%; /* 12.5% mult by 160px == 20px */
    }

  div.bar
    {
      margin-bottom: 0%; /* 0px */
      margin-left: 31.25%; /* 31.25% mult by 160px == 50px */
      margin-right: 0%; /* 0px */
      margin-top: 55.625%; /* 20px + 50px + 19px == 89px ; 89px divided by 160px == 55.625% */
    }

  div#reference
    {
      margin-top: 1em;
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
