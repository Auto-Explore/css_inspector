# css/css-writing-modes/percent-padding-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/percent-padding-vrl-002.xht"
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
      writing-mode: vertical-rl;
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
      padding-bottom: 10%;
      padding-left: 25%;
      padding-right: 50%;
      padding-top: 2.5%;
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
