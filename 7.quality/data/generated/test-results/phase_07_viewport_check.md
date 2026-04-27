# Phase 07: Viewport Humanization - Verification Checklist

**Date:** 2026-04-10
**Phase:** 07 - Viewport Humanization
**Status:** 📝 CHECKLIST CREATED - REQUIRES MANUAL TESTING

---

## 1. Verification Requirements

This phase requires **manual GUI testing** to verify viewport quality.

**Command to run:**
```bash
cargo run -p stratumx_editor_app --features desktop -- --gui
```

---

## 2. Viewport Quality Checklist

### On Launch (First Impression)

#### ✅ Visual Quality
- [ ] Sky gradient looks natural (not banded or glitchy)
- [ ] Sun disk is visible and properly positioned
- [ ] Clouds are visible (if weather is not Clear)
- [ ] Terrain is visible with proper texturing
- [ ] No black screen or missing elements
- [ ] No visual artifacts or z-fighting

#### ✅ Camera Defaults
- [ ] Camera is positioned to show horizon
- [ ] Camera is not inside terrain
- [ ] Camera is not too far away
- [ ] Camera angle shows both sky and terrain
- [ ] Initial view is aesthetically pleasing

#### ✅ Performance
- [ ] Viewport renders at acceptable framerate (>30 FPS)
- [ ] No stuttering or freezing
- [ ] Smooth camera movement (if testing controls)
- [ ] No memory leaks visible

---

## 3. Panel Usability Checklist

### Layout
- [ ] Panels don't overlap viewport excessively
- [ ] Panels are resizable
- [ ] Panels can be collapsed/expanded
- [ ] Panel layout is intuitive
- [ ] No panels blocking critical UI elements

### Functionality
- [ ] All visible controls are functional
- [ ] Sliders respond smoothly
- [ ] Buttons provide visual feedback
- [ ] Dropdowns work correctly
- [ ] Text inputs accept input

### Responsiveness
- [ ] UI responds immediately to input
- [ ] No lag between input and visual update
- [ ] Viewport updates when controls change
- [ ] Status messages appear when expected

---

## 4. Camera Control Checklist

### Mouse Controls
- [ ] Left-click drag rotates camera (if implemented)
- [ ] Right-click drag pans camera (if implemented)
- [ ] Scroll wheel zooms (if implemented)
- [ ] Camera doesn't flip or glitch
- [ ] Camera movement feels natural

### Keyboard Controls (if implemented)
- [ ] WASD movement works
- [ ] Camera speed is reasonable
- [ ] No camera clipping through terrain
- [ ] Camera stays oriented correctly

---

## 5. Environment Controls Checklist

### Time of Day
- [ ] Slider moves smoothly
- [ ] Sky color changes appropriately
- [ ] Sun position updates
- [ ] Changes are visible immediately
- [ ] No flickering or artifacts

### Weather
- [ ] Dropdown shows all weather options
- [ ] Selecting weather updates viewport
- [ ] Cloud coverage changes visibly
- [ ] Weather transitions look natural
- [ ] No crashes when changing weather

### Cloud Coverage
- [ ] Slider moves smoothly
- [ ] Cloud density changes visibly
- [ ] Changes are immediate
- [ ] No performance issues

---

## 6. Terrain Controls Checklist

### Terrain Panel
- [ ] Tool selection buttons work
- [ ] Brush settings sliders work
- [ ] Terrain info displays correctly
- [ ] Rebuild button works
- [ ] Import heightmap UI is accessible

### Terrain Interaction (if implemented)
- [ ] Clicking terrain shows position
- [ ] Brush preview visible (if implemented)
- [ ] Sculpting updates mesh (if implemented)
- [ ] Changes are visible immediately

---

## 7. Screenshot Evidence

### Required Screenshots

1. **Launch State**
   - Filename: `viewport_launch_state.png`
   - Shows: Initial viewport on launch
   - Verify: Sky, terrain, panels visible

2. **Time of Day - Morning**
   - Filename: `viewport_morning.png`
   - Shows: Time set to ~06:00
   - Verify: Dark blue sky, low sun

3. **Time of Day - Noon**
   - Filename: `viewport_noon.png`
   - Shows: Time set to ~12:00
   - Verify: Bright blue sky, high sun

4. **Time of Day - Evening**
   - Filename: `viewport_evening.png`
   - Shows: Time set to ~18:00
   - Verify: Orange/pink sky, low sun

5. **Weather - Clear**
   - Filename: `viewport_weather_clear.png`
   - Shows: Clear weather selected
   - Verify: Minimal clouds

6. **Weather - Storm**
   - Filename: `viewport_weather_storm.png`
   - Shows: Storm weather selected
   - Verify: Heavy clouds, dark sky

7. **Panel Layout**
   - Filename: `viewport_panel_layout.png`
   - Shows: All panels visible
   - Verify: No overlap, good layout

8. **Full UI**
   - Filename: `viewport_full_ui.png`
   - Shows: Complete editor interface
   - Verify: Professional appearance

---

## 8. Usability Issues to Check

### Common Problems

#### Visual Issues
- [ ] Banding in sky gradient
- [ ] Flickering elements
- [ ] Z-fighting on terrain
- [ ] Missing textures
- [ ] Incorrect colors
- [ ] Artifacts at horizon

#### Performance Issues
- [ ] Low framerate (<30 FPS)
- [ ] Stuttering
- [ ] Memory leaks
- [ ] CPU/GPU spikes
- [ ] Slow UI response

#### Layout Issues
- [ ] Panels too large
- [ ] Panels overlap viewport
- [ ] Text cut off
- [ ] Buttons too small
- [ ] Poor spacing

#### Functional Issues
- [ ] Controls don't work
- [ ] Viewport doesn't update
- [ ] Crashes on interaction
- [ ] Error messages unclear
- [ ] Missing features

---

## 9. Acceptance Criteria

### Must Pass:
- ✅ Viewport looks human-grade on launch
- ✅ Camera defaults are sane
- ✅ Panels don't destroy usability
- ✅ No critical visual bugs
- ✅ Performance is acceptable

### Should Pass:
- ✅ All environment controls work
- ✅ Time of day changes look good
- ✅ Weather changes are visible
- ✅ UI is responsive
- ✅ Layout is professional

### Nice to Have:
- ✅ Camera controls feel natural
- ✅ Terrain interaction works
- ✅ Advanced features functional
- ✅ Polish and refinement

---

## 10. Testing Procedure

### Step 1: Clean Build
```bash
cargo clean
cargo build -p stratumx_editor_app --features desktop --release
```

### Step 2: Launch Editor
```bash
cargo run -p stratumx_editor_app --features desktop --release -- --gui
```

### Step 3: Initial Observation
- Wait for editor to fully load
- Observe initial viewport state
- Check for any errors in console
- Take screenshot of launch state

### Step 4: Test Environment Controls
- Move time of day slider through full range
- Try each weather option
- Adjust cloud coverage
- Observe viewport updates
- Take screenshots of key states

### Step 5: Test Camera (if implemented)
- Try mouse controls
- Try keyboard controls (if any)
- Verify camera doesn't glitch
- Check camera limits

### Step 6: Test Panels
- Resize panels
- Collapse/expand panels
- Check all controls work
- Verify layout doesn't break

### Step 7: Performance Check
- Monitor framerate
- Check CPU/GPU usage
- Look for memory leaks
- Test for extended period (5+ minutes)

### Step 8: Document Issues
- List any visual problems
- Note any usability issues
- Record any crashes
- Capture error messages

---

## 11. Issue Reporting Template

### Issue Format:
```
**Issue:** [Brief description]
**Severity:** [Critical/High/Medium/Low]
**Category:** [Visual/Performance/Usability/Functional]
**Steps to Reproduce:**
1. [Step 1]
2. [Step 2]
3. [Step 3]
**Expected:** [What should happen]
**Actual:** [What actually happens]
**Screenshot:** [If applicable]
```

---

## 12. Evidence Report Template

### After Testing:

```markdown
# Phase 07: Viewport Humanization - Evidence Report

**Date:** [Date]
**Tester:** [Name]
**Build:** Release
**Platform:** Windows

## Test Results

### Launch Quality: [PASS/FAIL]
- Initial viewport: [Description]
- Camera position: [Description]
- Visual quality: [Description]

### Environment Controls: [PASS/FAIL]
- Time of day: [PASS/FAIL]
- Weather: [PASS/FAIL]
- Clouds: [PASS/FAIL]

### Usability: [PASS/FAIL]
- Panel layout: [PASS/FAIL]
- Controls: [PASS/FAIL]
- Responsiveness: [PASS/FAIL]

### Performance: [PASS/FAIL]
- Framerate: [X FPS]
- CPU usage: [X%]
- Memory: [X MB]

## Issues Found
[List issues using template above]

## Screenshots
[Attach screenshots]

## Overall Assessment
[PASS/FAIL with explanation]
```

---

## 13. Next Steps

### If Tests Pass:
1. ✅ Create evidence report with screenshots
2. ✅ Mark Phase 07 as COMPLETE
3. ✅ Continue to Phase 08

### If Tests Fail:
1. ⚠️ Document all issues
2. ⚠️ Prioritize fixes
3. ⚠️ Fix critical issues
4. ⚠️ Retest
5. ⚠️ Update evidence report

---

## 14. Deferred Testing Option

### If GUI Testing Not Possible Now:

**Option:** Mark phase as "REQUIRES MANUAL TESTING"

**Rationale:**
- Automated testing of visual quality is difficult
- Requires human judgment
- Can be done in separate testing session
- Doesn't block other phases

**If deferring:**
1. ✅ Checklist created (this document)
2. ✅ Testing procedure documented
3. ✅ Evidence template provided
4. ✅ Can proceed with other phases
5. 📝 Schedule manual testing session

---

**Report Generated:** 2026-04-10
**Phase Status:** 📝 CHECKLIST CREATED - MANUAL TESTING REQUIRED
**Next Phase:** Phase 08 - Open/Save/Product Shell Closure (can proceed)
**Testing Time:** 30-60 minutes (when performed)
