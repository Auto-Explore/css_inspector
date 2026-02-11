# css/css-writing-modes/border-vrl-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/border-vrl-006.xht"
}
```

## style[0]

```css
<![CDATA[
  .outer
    {
      border: blue solid 3px;
      width: 200px;
    }

  hr
    {
      background-color: blue;
      border: transparent none 0px;
      height: 9px;
      margin: 0px;
    }

  .inner
    {
      background-color: transparent;
      height: 50px; /* necessary, otherwise inner blocks must grow as tall as the height of viewport */
    }

  .foo
    {
      border-bottom: blue solid 5px;
      border-left: blue solid 100px;
      border-right: blue solid 50px;
      border-top: blue solid 20px;
      writing-mode: vertical-rl;
    }

  .bar
    {
      border-bottom: blue solid 20px;
      border-left: blue solid 50px;
      border-right: blue solid 100px;
      border-top: blue solid 5px;
      writing-mode: vertical-rl;
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
