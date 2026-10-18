
#include <cstdint>
#include <string>
#include <vector>

#pragma once

enum class Compliment {
    Cleanliness,
    Politeness,
    ComfortableDriving,
    Speed,
    NoComments
};

enum class DriverTier {
    Platinum,
    Gold,
    Standard,
    Risk
};

struct TierMeta {
    std::string name;
    std::string badge;
    double dispatchBonusPercent;
    bool isWarningIssued;
};

struct RatingInput {
    std::string driverId;
    std::string driverName;
    double currentRating = 5.0;
    int totalPreviousTrips = 0;
    int tripStars = 5;
    std::vector<Compliment> compliments;
    std::string comment;
};

struct RatingResult {
    double oldRating = 0.0;
    double newRating = 0.0;
    double ratingDelta = 0.0;
    int updatedTripCount = 0;
    DriverTier tier = DriverTier::Standard;
    TierMeta tierMeta;
};

void printRatingReceipt(const RatingInput &input, const RatingResult &result);
bool inputRatingData(RatingInput &input);
RatingResult processRatingData(const RatingInput &input);
