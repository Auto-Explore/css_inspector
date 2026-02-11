# css/css-writing-modes/abs-pos-non-replaced-icb-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/abs-pos-non-replaced-icb-vrl-002.xht"
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
      border-left: green solid 25px;
      border-right: green solid 75px;
      height: 100px;
      left: auto;
      position: absolute;
      right: auto;
      width: auto;
      writing-mode: vertical-rl;
    }

  /*
  "
  If all three of 'left', 'width', and 'right' are 'auto': First set any 'auto' values for 'margin-left' and 'margin-right' to 0. Then, if the 'direction' property of the element establishing the static-position containing block is 'ltr' set 'left' to the static position and apply rule number three below; otherwise, set 'right' to the static position and apply rule number one below.
  (...)
  3. 'width' and 'right' are 'auto' and 'left' is not 'auto', then the width is shrink-to-fit . Then solve for 'right'
  "
  10.3.7 Absolutely positioned, non-replaced elements
  http://www.w3.org/TR/CSS21/visudet.html#abs-non-replaced-width

  So:

            auto : left
        +
            0px : margin-left
        +
           25px : border-left-width
        +
            0px : padding-left
        +
           auto : width
        +
            0px : padding-right
        +
           75px : border-right-width
        +
            0px : margin-right
        +
           auto : right
        ====================
                : width of containing block (width of Initial Containing Block)

  becomes

            8px : left
        +
            0px : margin-left
        +
           25px : border-left-width
        +
            0px : padding-left
        +
            0px : width (shrink-to-fit)
        +
            0px : padding-right
        +
           75px : border-right-width
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
