# css/css-writing-modes/ortho-htb-alongside-vrl-floats-010.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/ortho-htb-alongside-vrl-floats-010.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

  div
    {
      width: 100px;
    }

  div#first-blue-float
    {
      background-color: blue;
      clear: left;
      float: left;
      height: 25%;
    }

  div#second-olive-float-with-clear
    {
      background-color: olive;
      clear: left;
      float: left;
      height: 50%;
    }

  div#third-fuchsia-float-with-clear
    {
      background-color: fuchsia;
      clear: left;
      float: left;
      height: 75%;
    }

  div#orange-horiz-tb
    {
      background-color: orange;
      height: 50%;
      writing-mode: horizontal-tb;
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
