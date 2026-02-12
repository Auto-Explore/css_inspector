# css/css-writing-modes/percent-padding-vlr-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/percent-padding-vlr-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div.outer
    {
      background-color: blue;
      border: blue solid 3px;
      width: 200px;
    }

  div.inner
    {
      background-color: transparent;
      height: 50px;
      writing-mode: vertical-lr;
    }

  div.foo
    {
      padding-bottom: 2.5%; /* 5px */
      padding-left: 50%; /* 100px */
      padding-right: 25%; /* 50px */
      padding-top: 10%; /* 20px */
    }

  hr
    {
      background-color: transparent;
      border: transparent none 0px;
      height: 3px;
      margin: 3px auto;
    }

  div.bar
    {
      padding-bottom: 10%; /* 20px */
      padding-left: 25%; /* 50px */
      padding-right: 50%; /* 100px */
      padding-top: 2.5%; /* 5px */
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
