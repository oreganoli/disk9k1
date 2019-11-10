$(document).ready(function () {
    let login_form = $(".login_form");
    let reg_form = $(".registration_form");
    let login_button = $(".login");
    let reg_button = $(".register");
    login_form.hide();
    reg_form.hide();
    login_button.click(function () {
        login_form.show();
        login_button.hide();
    });
    reg_button.click(function () {
        reg_form.show();
        reg_button.hide();
    });
});