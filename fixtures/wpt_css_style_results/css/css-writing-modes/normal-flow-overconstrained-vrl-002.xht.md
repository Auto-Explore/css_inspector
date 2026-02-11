# css/css-writing-modes/normal-flow-overconstrained-vrl-002.xht

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/normal-flow-overconstrained-vrl-002.xht"
}
```

## style[0]

```css
<![CDATA[
  html
    {
      writing-mode: vertical-rl;
    }

  body#containing-block
    {
      background-image: url("support/bg-red-3col-3row-320x320.png");
      background-position: -152px 8px;
      /* first value represents the horizontal position and the second represents the vertical position */

      /*
         8px (body's margin-left)
      -
        80px (4th column width)
      -
        80px (3rd column width)
      ========
      -152px
      */
      background-repeat: no-repeat;
      direction: ltr;
      height: 320px;
      margin: 8px;
    }

  p
    {
      margin-left: 16px;
      margin-right: 16px;
    }

  div
    {
      background-color: green;
      margin-top: 160px;
      border-top: green solid 20px;
      padding-top: 20px;
      height: 0px;
      padding-bottom: 20px;
      border-bottom: green solid 20px;
      margin-bottom: -160px;

      margin-right: 96px; /* 80px + 16px necessary so that we can reuse an already created and available reference file */
      width: 80px;
    }

/*
"
Layout calculation rules (such as those in CSS2.1, Section 10.3) that apply to the horizontal dimension in horizontal writing modes instead apply to the vertical dimension in vertical writing modes.
"
7.1 Principles of Layout in Vertical Writing Modes
http://www.w3.org/TR/css-writing-modes-3/#vertical-layout

So here, *-top and *-bottom properties are input into the §10.3.3 algorithms where *-top properties refer to *-left properties in the layout rules and where *-bottom properties refer to *-right properties in the layout rules.

'margin-top' + 'border-top-width' + 'padding-top' + 'height' + 'padding-bottom' + 'border-bottom-width' + 'margin-bottom' = height of containing block

So:

     160px : margin-top
  +
      20px : border-top-width
  +
      20px : padding-top
  +
       0px : height
  +
      20px : padding-bottom
  +
      20px : border-bottom-width
  +
    -160px : margin-bottom
   ========================
      80px : while the height of containing block of div is 320px

So, here, the specified value of 'margin-bottom' is ignored and the value is calculated so as to make the equality true

     160px : margin-top
  +
      20px : border-top-width
  +
      20px : padding-top
  +
       0px : height
  +
      20px : padding-bottom
  +
      20px : border-bottom-width
  +
   (solve) : margin-bottom
   ========================
     320px : height of containing block

And so computed margin-bottom value must be 80px .
*/

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
