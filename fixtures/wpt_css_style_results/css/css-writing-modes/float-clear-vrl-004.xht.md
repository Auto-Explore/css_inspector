# css/css-writing-modes/float-clear-vrl-004.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-clear-vrl-004.xht"
}
```

## style[0]

```css
<![CDATA[
  div#wrapper
    {
      background: red url("support/pattern-rg-rg-100x100.png");
      color: transparent;
      font: 50px/1 Ahem; /* computes to 50px/50px */
      height: 2em; /* computes to 100px */
      width: 2em; /* computes to 100px */
      writing-mode: vertical-rl;
    }

  div#floated-left
    {
      float: left;
    }

  div#cleared-left
    {
      background-color: green;
      clear: left;
      width: 1em;
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
