$(document).ready(function () {
    let login_form = $(".login_form");
    let reg_form = $(".registration_form");
    let login_button = $(".login");
    let reg_button = $(".register");
    let or = $(".or");
    login_form.hide();
    reg_form.hide();
    login_button.click(function () {
        or.hide();
        login_form.show();
        login_button.hide();
        reg_button.hide();
    });
    reg_button.click(function () {
        or.hide();
        reg_form.show();
        reg_button.hide();
        login_button.hide();
    });
});