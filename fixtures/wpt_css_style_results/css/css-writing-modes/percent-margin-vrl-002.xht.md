# css/css-writing-modes/percent-margin-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/percent-margin-vrl-002.xht"
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
      background-color: yellow;
      height: 50px;
      width: 50px;
      writing-mode: vertical-rl;
    }

  div.foo
    {
      margin-bottom: 2.5%; /* 5px ; collapses with hr's margin-top */
      margin-left: 50%; /* 100px */
      margin-right: 25%; /* 50px */
      margin-top: 10%; /* 20px */
    }

  hr
    {
      background-color: transparent;
      border: transparent none 0px;
      height: 3px;
      margin: 0.5em auto;
    }

  div.bar
    {
      margin-bottom: 10%; /* 20px */
      margin-left: 25%; /* 50px */
      margin-right: 50%; /* 100px */
      margin-top: 2.5%; /* 5px ; collapses with hr's margin-bottom */
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
