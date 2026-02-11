# css/css-writing-modes/float-clear-vlr-009.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/float-clear-vlr-009.xht"
}
```

## style[0]

```css
<![CDATA[
  div#wrapper
    {
      background: red url("support/pattern-gr-gr-100x100.png");
      font-size: 50px;
      height: 2em; /* computes to 100px */
      width: 2em; /* computes to 100px */
      writing-mode: vertical-lr;
    }

  div#container
    {
      border-right: green solid 1em;
      width: auto; /* Very important: we want div#container to be able to shrink-wrap */
    }

  div#floated-left
    {
      float: left;
      height: 50%; /* computes to 50px */
      width: 1em; /* computes to 50px */
    }

  div#floated-right
    {
      float: right;
      height: 50%; /* computes to 50px */
      width: 1em; /* computes to 50px */
    }

  div#clearing
    {
      clear: both;
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
