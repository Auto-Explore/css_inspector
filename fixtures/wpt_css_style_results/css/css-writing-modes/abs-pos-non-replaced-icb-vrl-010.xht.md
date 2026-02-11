# css/css-writing-modes/abs-pos-non-replaced-icb-vrl-010.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-icb-vrl-010.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      direction: ltr;
    }

  div#green-overlapping-test
    {
      background-color: green;
      border-left: green solid 35px;
      border-right: green solid 15px;
      height: 100px;
      left: auto;
      position: absolute;
      right: auto;
      width: 50px;
      writing-mode: vertical-rl;
    }

  /*
  "
  2. 'left' and 'right' are 'auto' and 'width' is not 'auto', then if the 'direction' property of the element establishing the static-position containing block is 'ltr' set 'left' to the static position (...) Then solve for (...) 'right' (if 'direction' is 'ltr').
  "
  10.3.7 Absolutely positioned, non-replaced elements
  http://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width

  So:

           auto : left
        +
            0px : margin-left
        +
           35px : border-left-width
        +
            0px : padding-left
        +
           50px : width
        +
            0px : padding-right
        +
           15px : border-right-width
        +
            0px : margin-right
        +
           auto : right
        ====================
                : width of containing block (width of Initial Containing Block)

  becomes

            8px : left (set to static position)
        +
            0px : margin-left
        +
           35px : border-left-width
        +
            0px : padding-left
        +
           50px : width
        +
            0px : padding-right
        +
           15px : border-right-width
        +
            0px : margin-right
        +
          solve : right
        ====================
                : width of containing block (width of Initial Containing Block)
  */

  div#red-overlapped-reference
    {
      background-color: red;
      height: 100px;
      width: 100px;
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
