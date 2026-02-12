# css/css-writing-modes/float-clear-vrl-006.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-clear-vrl-006.xht"
}
```

## style[0]

```css
<![CDATA[
  div#wrapper
    {
      background: red url("support/pattern-rg-rg-100x100.png");
      font-size: 50px;
      height: 2em; /* computes to 100px */
      width: 2em; /* computes to 100px */
      writing-mode: vertical-rl;
    }

  div#container
    {
      border-left: green solid 1em;
      width: auto; /* Very important: we want div#container to be able to shrink-wrap */
    }

  div.floated-left
    {
      float: left;
      height: 50%; /* computes to 50px */
      width: 1em; /* computes to 50px */
    }

  div#clearing
    {
      clear: left;
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
