# css/css-fonts/font-size-adjust-003.xht

```json
{
  "format_version": 3,
  "file": "css/css-fonts/font-size-adjust-003.xht"
}
```

## style[0]

```css
<![CDATA[
  div.wrapper
    {
      font-size: 200px;
      line-height: 1;
    }
  /*
  The 'line-height: 1' declaration
  is not part of the test. Its
  sole purpose is to not increase
  document box height unneedlessly.
  */

  img
    {
      height: 1px;
      margin-right: -100%;
      width: 100%;
    }

  img.top-x-height
    {
      vertical-align: 90px;
    }
  /*  200px mult by 0.450 == 90px  */

  img.baseline
    {
      vertical-align: -1px;
    }
  /*
  We want the bottom green line to be flush with the bottom of
  glyphs and not be overlapped by the bottom of glyphs. Therefore
  this 'vertical-align: -1px' declaration.
  */

  span.test
    {
      font-family: uninstalled, notavailable, bogus, "DejaVu Sans", "Oxygen-Sans", "Liberation Sans", Verdana, Tahoma;
  /*
  This test presumes that the tester will have at least
  one of the following font installed on his/her operating system:
  "DejaVu Sans", "Oxygen-Sans", "Liberation Sans", Verdana, Tahoma
  */
      font-size-adjust: 0.450;
    }

  /*
  List of 5 font faces with relatively big aspect values

  DejaVu Sans' aspect value     == 0.547
  Oxygen-Sans' aspect value     == 0.540
  Liberation Sans' aspect value == 0.530
  (DejaVu Sans, Oxygen-Sans and Liberation Sans are font
  faces very often installed under Linux Debian-distributions)

  Verdana's aspect value        == 0.545
  Tahoma's aspect value         == 0.545
  (Verdana and Tahoma are font faces very often pre-installed
  under Windows operating systems.)
  */

  span.reference
    {
      font-family: Rachana, "Nimbus Roman No9 L", "Times New Roman";
    }

  /*
  Rachana's aspect value            == 0.450
  Nimbus Roman No9 L's aspect value == 0.450
  (Rachana and Nimbus Roman No9 L are font
  faces often installed under Linux Debian-distributions)

  Times New Roman's aspect value    == 0.448
  (Times New Roman is a font face is very often pre-installed
  under Windows operating systems.)
  */

  /*
  This test presumes that the tester will have at least
  one of the following font installed on his/her operating system:
  Rachana, "Nimbus Roman No9 L", "Times New Roman"
  */
  ]]>
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
