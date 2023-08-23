const { invoke } = window.__TAURI__.tauri;

let selectProfileEl;
let restartButtonEl;
let newProfileInputEl;
let createAndRestartFormEl;
let currentProfileSpanEl;

window.addEventListener("DOMContentLoaded", async () => {

  const currentProfile = await invoke("get_active_profile", {});
  currentProfileSpanEl = document.querySelector("#current-profile");
  currentProfileSpanEl.innerHTML = currentProfile;
  currentProfileSpanEl.style["background"] = "#e5e5e5";
  currentProfileSpanEl.style["border-radius"] = "5px";
  currentProfileSpanEl.style["padding"] = "3px 5px";

  const allProfiles = await invoke("get_existing_profiles", {});
  selectProfileEl = document.querySelector("#profile-selector");
  console.log("all profiles: ", allProfiles);
  allProfiles.forEach((profile) => {
    const option = document.createElement("option");
    option.setAttribute("name", profile);
    option.innerHTML = profile;
    selectProfileEl.appendChild(option);
  });

  restartButtonEl = document.querySelector("#restart-button");
  restartButtonEl.addEventListener('click', async () => {
    await invoke("set_active_profile", { profile: selectProfileEl.value });
    await invoke("restart", {});
    const currentProfile = selectProfileEl.value;
    console.log("Current value: ", currentProfile);
    // restart with the given profile
  })

  newProfileInputEl = document.querySelector("#new-profile-input");
  createAndRestartFormEl = document.querySelector("#create-and-restart-form");

  newProfileInputEl.addEventListener("change", () => {
    if (allProfiles.includes(newProfileInputEl.value)) {
      newProfileInputEl.setCustomValidity("A profile with this name already exists.");
    } else {
      newProfileInputEl.setCustomValidity("");
    }
  });
  createAndRestartFormEl.addEventListener('submit', async (e) => {
    e.preventDefault();
    await invoke("set_active_profile", { profile: newProfileInputEl.value });
    await invoke("restart", {});
  })
});
