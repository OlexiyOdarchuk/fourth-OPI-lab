#pragma once

#include <cstdint>

// Грошова сума в копійках (1 грн = 100 коп.)
using Money = std::int64_t;

// Структури визначені в cxx-мості
struct TripRequest;
struct FareCalculationResult;

FareCalculationResult processFareCalculation(const TripRequest &req);
